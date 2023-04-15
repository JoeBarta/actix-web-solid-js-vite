import { render } from 'solid-js/web'
import { Button } from './components/Button'

function App() {
	const name = 'Rusty Solid'

	return (
		<>
			<div>Hello {name}!</div>
			<Button />
		</>
	)
}

const el = document.getElementById('app')
if (!el) throw new Error('Cannot find element #app to render app')

render(() => <App />, el)
