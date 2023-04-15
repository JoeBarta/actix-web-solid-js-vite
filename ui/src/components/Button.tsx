import { createResource, createSignal, createEffect, Show, For } from 'solid-js'

interface User {
	id: number
	name: string
}

const fetchUsers = async () => {
	const result = await fetch('/users')

	if (!result.ok) {
		throw new Error('Failed to fetch users')
	}

	const json = await result.json()
	return json.users.map((user: any) => ({ id: user.id, name: user.name }))
}

export function Button() {
	const [trigger, setTrigger] = createSignal(false)
	// @ts-expect-error - what is typescript anyway
	const [users] = createResource<User[]>(trigger, fetchUsers)

	createEffect(() => {
		console.log('loading', users.loading)
	})

	return (
		<>
			<button onClick={() => setTrigger(true)}>Fetch me a list of my friends darling</button>
			<Show when={!users.error} fallback={<div>Oh no! An error!</div>}>
				<ul>
					<For each={users()}>
						{(user, i) => (
							<li>
								{i() + 1}: {user.name}
							</li>
						)}
					</For>
				</ul>
			</Show>
		</>
	)
}
