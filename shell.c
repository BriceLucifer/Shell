#include <sys/wait.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/*
 * Function Declarations for buildinshell commands
 */

int bsh_cd(char **args);
int bsh_help(char **args);
int bsh_exit(char **args);

/*
 * Function name array
 */
char *buildin_str[] = {"cd","help","exit"};

/* 
 * Function Pointer as array
 */
int (*buildin_func[])(char **) = {&bsh_cd,&bsh_help,&bsh_exit};

/*numbers of bsh*/
int bsh_num_buildin(){
	return sizeof(buildin_str) / sizeof(char*);
}

/*
 * function cd
 */
int bsh_cd(char **args){
	if(args[1] == NULL){
		fprintf(stderr,"bsh : expected arguments to \"cd\" \n");
	}else{
		if(chdir(args[1]) != 0){
			perror("bsh");
		}
	}

	return 1;
}

/*
 * help buildin_func
 */
int bsh_help(char **args){
	printf("Brice first Shell: bsh\n");
	printf("Type Program you want and Hit <CR>\n");
	printf("The Following is buildin_functions\n");

	// print function which is buildinshell
	for(int i = 0; i < bsh_num_buildin(); i++){
		printf("%s\n",buildin_str[i]);
	}
	
	printf("use man commands for information on other programs.\n");
	return 1;
}

/*
 * 退出程序
 */
int bsh_exit(char **args){
	return 0;
}

/*
 * launch a Program and wait to terminate.
 * return always 1 to continue execution
 */
int bsh_launch(char **args){
	pid_t pid, wpid;
	int status;

	pid = fork();

	// process control
	if(pid == 0){
		//child process
		if (execvp(args[0],args) == -1){
			perror("bsh");
		}
		exit(EXIT_FAILURE);
	}else if(pid < 0){
		perror("bsh");	
	}else{
		do{
			wpid = waitpid(pid,&status,WUNTRACED);
		}while(!WIFEXITED(status) && !WIFSIGNALED(status));
	}

	return 1;
}

int bsh_execute(char **args){

	if(args[0] == NULL){
		return 1;
	}

	for(int i = 0 ; i< bsh_num_buildin(); i++){
		if(strcmp(args[0],buildin_str[i]) == 0){
			return(*buildin_func[i])(args);
		}
	}

	return bsh_launch(args);
}

#define BSH_RL_BUFSIZE 1024

char *bsh_read_line(){
	int bufsize = BSH_RL_BUFSIZE;
	int position = 0;
	char *buffer = malloc(sizeof(char) * bufsize);

	if(!buffer){
		fprintf(stderr,"bsh: allocation error\n");
		exit(EXIT_FAILURE);
	}

	while(1){
		int c = getchar();

		if(c == EOF || c == '\n'){
			buffer[position] = '\0';
			return buffer;
		}else{
			buffer[position] = c;
		}
		position++;

		if(position >= bufsize){
			bufsize += BSH_RL_BUFSIZE;
			buffer = realloc(buffer,bufsize);
			if(!buffer){
				fprintf(stderr,"bsh: allocation error\n");
				exit(EXIT_FAILURE);
			}
		}
	}
}

#define BSH_TOK_BUFSIZE 64
#define BSH_TOK_DELIM " \t\r\n\a"

char **bsh_spilt_line(char *line){
	int bufsize = BSH_TOK_BUFSIZE;
	int position = 0;
	char **tokens = malloc(bufsize * sizeof(char*));
	char *token;

	if(!tokens){
		fprintf(stderr,"bsh: allocation error\n");
		exit(EXIT_FAILURE);
	}

	token = strtok(line,BSH_TOK_DELIM);
	while(token != NULL){
		tokens[position] = token;
		position++;

		if(position >= bufsize){
			bufsize += BSH_TOK_BUFSIZE;
			tokens = realloc(tokens,bufsize * sizeof(char*));
		if(!tokens){
			fprintf(stderr,"bsh: allocation error\n");
			exit(EXIT_FAILURE);
		}
		}

		token = strtok(NULL,BSH_TOK_DELIM);

	}
	tokens[position] = NULL;
	return tokens;
}

void bsh_loop(){
	char *line;
	char **args;
	int status;

	do{
		printf("> ");
		line = bsh_read_line();
		args = bsh_spilt_line(line);
		status = bsh_execute(args);

		free(line);
		free(args);
	}while(status);
}

int main(int argc,char **argv){
	// load config files
	// run command loop
	bsh_loop();

	// perform shutdown/cleanup
	return EXIT_SUCCESS;
}
