import { useMutation } from '@tanstack/react-query'
import { useState } from 'react'
import Card from '../../../base/Card'
import Form, { FormSubmitButton, FormTextField } from '../../../base/Form'
import Spinner from '../../../base/Spinner'
import {ZERO_ADDRESS} from "../../../../global/constants";


function FundEthCard() {

  const [accountToFund, setAccountToFund] = useState(ZERO_ADDRESS)

  const fundEthMutation = useMutation(
    ["fundEth", accountToFund],
    async () => setTimeout(() => console.log("funded account with eth"), 3_000),
    {
      onError: (e: any) => alert(e.message)
    }
  )

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault()
    fundEthMutation.mutate()
  }

  return <Card title={'Fund Eth'} state={'UNDER_CONSTRUCTION'}>
    <Form onSubmit={handleSubmit}>
      <FormTextField label={'Account'} value={accountToFund}
                     onChange={(event) => setAccountToFund(event.target.value)} />
      <FormSubmitButton>{fundEthMutation.isLoading ? <Spinner /> : 'Fund'}</FormSubmitButton>
    </Form>
  </Card>
}


export default FundEthCard
