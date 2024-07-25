/**
 * nx python executor
 */


import { ExecutorContext } from '@nrwl/devkit';
import { exec } from 'child_process';

export interface PythonExecutorOptions {
  command: string;
  args?: string[];
}

export default async function pythonExecutor(
  options: PythonExecutorOptions,
  context: ExecutorContext
) {
    return new Promise<{ success: boolean }>((resolve) => {
        const args = options.args ? options.args.join(' ') : '';
        const command = `python ${options.command} ${args}`;
        
        exec(command, { cwd: context.cwd }, (error, stdout, stderr) => {
            console.log(stdout);
            console.error(stderr);
            resolve({ success: !error });
        });
    });
}
