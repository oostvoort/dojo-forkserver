import { useState } from 'react'
import Card from '../../../base/Card'
import {useMutation} from '@tanstack/react-query'
import Slider from "../../../base/Slider";
import Button from "../../../base/Button";
import {manipulateBlock} from "../../../../global/utils";
import {useLatestBlock} from "../../../../hooks/useBlockInformation";

function IncreaseTimeCard() {

  const [timeToIncrease, setTimeToIncrease] = useState(1_000)
  const blockInformation = useLatestBlock()

  const increaseTimeMutation = useMutation(
    ["increaseTime", timeToIncrease],
    async () => {
      await manipulateBlock("IncreaseTime", timeToIncrease)
      await blockInformation.refetch()
    },
    {
      onError: (e: any) => alert(e.message)
    })

  const blockTime = blockInformation.data?.timestamp ?? 0

  return <Card title={`Increase Time (${blockTime})`}>
    <div className={'flex flex-col gap-4'}>
      <text className='text-white-700 font-bold'>Seconds</text>
      <Slider min={1} max={10_000} step={1} value={timeToIncrease} onChange={setTimeToIncrease} />
      <Button width={'full'} isLoading={increaseTimeMutation.isLoading}
              onClick={() => increaseTimeMutation.mutate()}>Increase Time</Button>
    </div>
  </Card>
}


export default IncreaseTimeCard
