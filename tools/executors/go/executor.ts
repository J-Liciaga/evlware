/**
 * nx golang executor
 */

import { ExecutorContext } from '@nrwl/devkit';
import { exec } from 'child_process';

export interface GolangExecutorOptions {
  command: string;
  args?: string[];
}

export default async function golangExecutor(
  options: GolangExecutorOptions,
  context: ExecutorContext
) {
    return new Promise<{ success: boolean }>((resolve) => {
        const args = options.args ? options.args.join(' ') : '';
        const command = `go ${options.command} ${args}`;
        
        exec(command, { cwd: context.cwd }, (error, stdout, stderr) => {
            console.log(stdout);
            console.error(stderr);
            resolve({ success: !error });
        });
    });
}
