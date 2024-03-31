export interface Task {
	id: number,
	name: string,
	description: string,
	estimatedPomodoros: number,
	completedPomodoros: number,
	done: boolean,
}
