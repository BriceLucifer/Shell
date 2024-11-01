#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#include <windows.h>
#include <process.h> // for _spawnlp
#include <direct.h>
#define fork() -1 // no fork in Windows
#define chdir _chdir // Define chdir as _chdir for Windows
#else
#include <unistd.h> // For chdir and fork
#include <sys/wait.h>
#endif

// Function Declarations for builtinshell commands
int bsh_cd(char **args);
int bsh_help(char **args);
int bsh_exit(char **args);

// Function name array
char *buildin_str[] = {"cd", "help", "exit"};

// Function Pointer as array
int (*buildin_func[])(char **) = {&bsh_cd, &bsh_help, &bsh_exit};

// Number of built-in commands
int bsh_num_buildin() {
    return sizeof(buildin_str) / sizeof(char *);
}

// Change directory
int bsh_cd(char **args) {
    if (args[1] == NULL) {
        fprintf(stderr, "bsh: expected arguments to \"cd\"\n");
    } else {
        #ifdef _WIN32
        if (_chdir(args[1]) != 0) {
            perror("bsh");
        }
        #else
        if (chdir(args[1]) != 0) {
            perror("bsh");
        }
        #endif
    }
    return 1;
}

// Help command
int bsh_help(char **args) {
    printf("Brice first Shell: bsh\n");
    printf("Type Program you want and Hit <CR>\n");
    printf("The Following is built-in functions:\n");

    for (int i = 0; i < bsh_num_buildin(); i++) {
        printf("%s\n", buildin_str[i]);
    }

    printf("use man commands for information on other programs.\n");
    return 1;
}

// Exit the program
int bsh_exit(char **args) {
    return 0;
}

// Launch a program and wait to terminate
int bsh_launch(char **args) {
#ifdef _WIN32
    if (_spawnlp(_P_WAIT, args[0], args[0], NULL) == -1) {
        perror("bsh");
    }
#else
    pid_t pid, wpid;
    int status;

    pid = fork();

    if (pid == 0) {
        // Child process
        if (execvp(args[0], args) == -1) {
            perror("bsh");
        }
        exit(EXIT_FAILURE);
    } else if (pid < 0) {
        perror("bsh");
    } else {
        do {
            wpid = waitpid(pid, &status, WUNTRACED);
        } while (!WIFEXITED(status) && !WIFSIGNALED(status));
    }
#endif
    return 1;
}

// Execute commands
int bsh_execute(char **args) {
    if (args[0] == NULL) {
        return 1;
    }

    for (int i = 0; i < bsh_num_buildin(); i++) {
        if (strcmp(args[0], buildin_str[i]) == 0) {
            return (*buildin_func[i])(args);
        }
    }

    return bsh_launch(args);
}

#define BSH_RL_BUFSIZE 1024

// Read a line from stdin
char *bsh_read_line() {
    int bufsize = BSH_RL_BUFSIZE;
    int position = 0;
    char *buffer = malloc(sizeof(char) * bufsize);

    if (!buffer) {
        fprintf(stderr, "bsh: allocation error\n");
        exit(EXIT_FAILURE);
    }

    while (1) {
        int c = getchar();

        if (c == EOF || c == '\n') {
            buffer[position] = '\0';
            return buffer;
        } else {
            buffer[position] = c;
        }
        position++;

        if (position >= bufsize) {
            bufsize += BSH_RL_BUFSIZE;
            buffer = realloc(buffer, bufsize);
            if (!buffer) {
                fprintf(stderr, "bsh: allocation error\n");
                exit(EXIT_FAILURE);
            }
        }
    }
}

#define BSH_TOK_BUFSIZE 64
#define BSH_TOK_DELIM " \t\r\n\a"

// Split the line into tokens
char **bsh_split_line(char *line) {
    int bufsize = BSH_TOK_BUFSIZE;
    int position = 0;
    char **tokens = malloc(bufsize * sizeof(char *));
    char *token;

    if (!tokens) {
        fprintf(stderr, "bsh: allocation error\n");
        exit(EXIT_FAILURE);
    }

    token = strtok(line, BSH_TOK_DELIM);
    while (token != NULL) {
        tokens[position] = token;
        position++;

        if (position >= bufsize) {
            bufsize += BSH_TOK_BUFSIZE;
            tokens = realloc(tokens, bufsize * sizeof(char *));
            if (!tokens) {
                fprintf(stderr, "bsh: allocation error\n");
                exit(EXIT_FAILURE);
            }
        }

        token = strtok(NULL, BSH_TOK_DELIM);
    }
    tokens[position] = NULL;
    return tokens;
}

// Main loop of the shell
void bsh_loop() {
    char *line;
    char **args;
    int status;

    do {
        printf("> ");
        line = bsh_read_line();
        args = bsh_split_line(line);
        status = bsh_execute(args);

        free(line);
        free(args);
    } while (status);
}

// Main function
int main(int argc, char **argv) {
    bsh_loop();
    return EXIT_SUCCESS;
}
