import { createSignal } from 'solid-js'

interface User {
	id: number
	name: string
}

export function Button() {
	const [users, setUsers] = createSignal<[] | User[]>([])

	const fetchUsers = async () => {
		const response = await fetch('/api/users')
		const data = await response.json()
		setUsers(data)
	}

	console.log(users)
	return (
		<>
			<button onClick={fetchUsers}>Fetch me a list of my friends darling</button>
			{users().map((user) => (
				<div>{user.name}</div>
			))}
		</>
	)
}
