#include <arpa/inet.h>
#include <netinet/in.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define PORT 8080
/* largest UDP payload: 65507 bytes */
#define BUF_SIZE 65536

static void die(const char *msg) {
    perror(msg);
    exit(1);
}

static int receive_file(int sock, const char *filename) {
    FILE *fp = fopen(filename, "wb");
    if (!fp) {
        perror(filename);
        return -1;
    }

    /* the whole file arrives in a single datagram */
    uint8_t buf[BUF_SIZE];
    ssize_t n = recvfrom(sock, buf, sizeof(buf), 0, NULL, NULL);
    if (n < 0)
        die("recvfrom");

    printf("  Received %zd bytes\n", n);
    if (n > 0)
        fwrite(buf, 1, n, fp);

    fclose(fp);
    return 0;
}

int main(void) {
    int sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (sock < 0)
        die("socket");

    struct sockaddr_in addr = {0};
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_ANY);
    addr.sin_port = htons(PORT);

    if (bind(sock, (struct sockaddr *)&addr, sizeof(addr)) < 0)
        die("bind");

    setbuf(stdout, NULL); /* keep logs live even when piped to a file */

    printf("File Sharing Server listening on port %d\n", PORT);

    char filename[1024];
    struct sockaddr_in client;
    socklen_t client_len = sizeof(client);

    for (;;) {
        /* wait for filename (first message from client) */
        ssize_t n = recvfrom(sock, filename, sizeof(filename) - 1, 0,
                             (struct sockaddr *)&client, &client_len);
        if (n < 0)
            die("recvfrom");
        filename[n] = '\0';

        char ip[INET_ADDRSTRLEN];
        inet_ntop(AF_INET, &client.sin_addr, ip, sizeof(ip));
        printf("\nIncoming file: %s from %s\n", filename, ip);

        /* send ACK to start file transfer */
        char ack = 1;
        if (sendto(sock, &ack, 1, 0, (struct sockaddr *)&client, client_len) < 0)
            die("sendto ack");

        if (receive_file(sock, filename) == 0)
            printf("File saved successfully\n");
        else
            printf("Error receiving file\n");
    }

    close(sock);
    return 0;
}