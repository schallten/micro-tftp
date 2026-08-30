#include <arpa/inet.h>
#include <netinet/in.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define PORT 8080
#define BUF_SIZE 65536

static void die(const char *msg) {
    perror(msg);
    exit(1);
}

static int send_file(int sock, const char *filename) {
    FILE *fp = fopen(filename, "rb");
    if (!fp) {
        perror(filename);
        return -1;
    }

    /* send filename first */
    if (send(sock, filename, strlen(filename), 0) < 0)
        die("send filename");

    /* wait for ACK before transfer */
    char ack;
    if (recv(sock, &ack, 1, 0) < 0)
        die("recv ack");

    /* read the whole file into memory and send it in one datagram */
    static uint8_t buf[BUF_SIZE];
    size_t n = fread(buf, 1, sizeof(buf), fp);
    fclose(fp);

    if (send(sock, buf, n, 0) < 0)
        die("send file");

    return 0;
}

int main(void) {
    int sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (sock < 0)
        die("socket");

    struct sockaddr_in local = {0};
    local.sin_family = AF_INET;
    local.sin_addr.s_addr = htonl(INADDR_ANY);
    local.sin_port = 0; /* ephemeral */
    if (bind(sock, (struct sockaddr *)&local, sizeof(local)) < 0)
        die("bind");

    struct sockaddr_in server = {0};
    server.sin_family = AF_INET;
    server.sin_addr.s_addr = inet_addr("127.0.0.1");
    server.sin_port = htons(PORT);
    if (connect(sock, (struct sockaddr *)&server, sizeof(server)) < 0)
        die("connect");

    printf("File sharing client\n");
    printf("Enter filename to send (or 'exit' to quit ) : \n");

    char filename[1024];
    for (;;) {
        printf("> ");
        fflush(stdout);
        if (!fgets(filename, sizeof(filename), stdin))
            break;

        /* strip trailing newline */
        size_t len = strlen(filename);
        if (len > 0 && filename[len - 1] == '\n')
            filename[len - 1] = '\0';

        if (strcmp(filename, "exit") == 0)
            break;
        if (filename[0] == '\0')
            continue;

        if (send_file(sock, filename) == 0)
            printf("File sent successfully\n");
        else
            printf("Error sending file\n");

        printf("\nEnter next filename : \n");
    }

    close(sock);
    return 0;
}