import React, {useState} from "react";
import useAccounts from "../../../../../hooks/useAccounts";
import Slider from "../../../../base/Slider";
import {toOverflowValue} from "../../../../../global/utils";

type PropsType = {
  title: string,
  value: string
}

const AccountDetail: React.FC<PropsType> = ({ title, value }) => {
  const handleClick = () => {
    navigator.clipboard.writeText(value)
  }

  const isOverflow = value.length > 25
  const overflownValue = toOverflowValue(value, 7, 5)

  return (
    <tr onClick={handleClick} className={"cursor-copy"}>
      <td className={"font-bold p-2"}>{title}</td>
      <td className={"p-2"}>{ isOverflow ? overflownValue : value }</td>
    </tr>
  )
}

const Accounts = () => {
  const [index, setIndex] = useState<number>(0)
  const accounts = useAccounts()
  const account = accounts?.data?.[index]

  return (
    <>
      <div className={"text-left"}>
        <span className={"font-bold"}> Predeployed Accounts: </span>{accounts?.data?.length ?? 0}
      </div>
      <div>
        <Slider min={0} max={(accounts?.data?.length ?? 0) - 1} step={1} value={index} onChange={setIndex} />
      </div>
      {!!account && (
        <div>
          <table className={"text-left"}>
            <AccountDetail title={"Address"} value={account.address} />
            <AccountDetail title={"Balance"} value={account.balance} />
            <AccountDetail title={"Private Key"} value={account.private_key} />
            <AccountDetail title={"Public Key"} value={account.public_key} />
          </table>
        </div>
      )}
    </>
  )
}

export default Accounts