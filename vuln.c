#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

// Vulnerable function with buffer overflow
void vulnerable_function(char *input) {
    char buffer[64];  // Small buffer
    printf("Input received: ");
    strcpy(buffer, input);  // Dangerous! No bounds checking
    printf("%s\n", buffer);
}


int main(int argc, char *argv[]) {
    printf("=== Vulnerable Test Program ===\n");
    printf("PID: %d\n", getpid());
    
    if (argc > 1) {
        printf("\n[Test 1] Buffer Overflow:\n");
        vulnerable_function(argv[1]);
    }
    
    printf("\nProgram completed.\n");
    return 0;
}
