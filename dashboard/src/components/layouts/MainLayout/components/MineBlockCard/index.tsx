import { useMutation } from '@tanstack/react-query'
import Card from '../../../../base/Card'
import Form from '../../../../base/Form'
import { useState } from 'react'
import MineBlock from "./MineBlock";
import Slider from "../../../../base/Slider";
import {manipulateBlock} from "../../../../../global/utils";
import {useLatestBlock} from "../../../../../hooks/useBlockInformation";

function MineBlockCard() {

  const [blocks, setBlocks] = useState(500)

  const blockInformation = useLatestBlock()

  const mineBlockMutation = useMutation(
    ["mineBlock"],
    async () => {
      await manipulateBlock("MineBlock", blocks)
      await blockInformation.refetch()
    },
    {
      onError: (e: any) => alert(e.message)
    })

  const blockNumber = blockInformation.data?.block_number ?? 0

  return <Card title={`Mine Block ${blockNumber ? `(${blockNumber})` : ''}`}>
    <Form>
      <MineBlock onMineBlock={mineBlockMutation.mutate} />
      <Slider min={1} max={10_000} step={1} value={blocks} onChange={setBlocks} />
    </Form>
  </Card>
}


export default MineBlockCard
