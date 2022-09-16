import React, { useState, useEffect } from "react"
import { Form, Grid } from "semantic-ui-react"
import KittyCards from "./KittyCards"
import { TxButton } from "./substrate-lib/components"
import { useSubstrateState } from "./substrate-lib"

export default function Main(props) {
	const [kittyCnt] = useState("创建小毛孩")
	const { api, keyring, currentAccount } = useSubstrateState()

	const [status, setStatus] = useState("")
	const [kitties, setKitties] = useState([])

	useEffect(async () => {
		await getDnas()
	}, [])

	const getDnas = async () => {
		const currentId = await api.query.kitties.nextKittyId()
		let kittyIds = [...Array(parseInt(currentId)).keys()]
		await api.query.kitties.kitties.multi(kittyIds, async (dnas) => {
			await api.query.kitties.kittyOwner.multi(kittyIds, (accountIds) => {
				let kitties = dnas.map((dna, id) => ({
					id,
					dna: dna.unwrap(),
					owner: keyring.encodeAddress(accountIds[id].unwrap())
				}))

				setKitties(kitties)
			})
		})
	}

	return (
		<Grid.Column width={16}>
			<h1>小毛孩</h1>
			<KittyCards setStatus={setStatus} kitties={kitties} accountPair={currentAccount} />
			<Form style={{ margin: "1em 0" }}>
				<Form.Field style={{ textAlign: "center" }}>
					<TxButton
						label={kittyCnt}
						type="SIGNED-TX"
						setStatus={setStatus}
						attrs={{
							palletRpc: "kitties",
							callable: "create",
							inputParams: [],
							paramFields: []
						}}
					/>
				</Form.Field>
			</Form>
			<div style={{ overflowWrap: "break-word" }}>{status}</div>
		</Grid.Column>
	)
}
