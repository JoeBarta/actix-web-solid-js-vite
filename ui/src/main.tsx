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

const elt = document.getElementById('app')
if (!elt) throw new Error('Cannot find element #root to render app')

render(() => <App />, elt)
