import { useState } from 'react'
import Card from '../../../base/Card'
import Form, { FormSubmitButton, FormTextField } from '../../../base/Form'
import Spinner from '../../../base/Spinner'
import {ZERO_ADDRESS} from "../../../../global/constants";
import {useMutation} from "@tanstack/react-query";

function FundTokenCard() {

  const [tokenAddress, setTokenAddress] = useState(ZERO_ADDRESS)
  const [accountToFund, setAccountToFund] = useState( ZERO_ADDRESS)

  const accountLabel = `Account (balance here)`

  const handleSubmit = () => {
    console.log("funding...")
  }

  const fundTokenMutation = useMutation(
    ['fundToken', tokenAddress, accountToFund],
    async () => setTimeout(() => console.log("funded account"), 3_000)
  )


  return <Card title={'Fund Token'} state={'INFEASIBLE'}>
    <Form onSubmit={handleSubmit}>
      <FormTextField label={accountLabel} value={accountToFund}
                     onChange={(event) => setAccountToFund(event.target.value)} />
      <FormTextField label={'Token'} value={tokenAddress} onChange={(event) => setTokenAddress(event.target.value)} />
      <FormSubmitButton>{fundTokenMutation.isLoading ? <Spinner /> : 'Fund'}</FormSubmitButton>
    </Form>
  </Card>
}


export default FundTokenCard
