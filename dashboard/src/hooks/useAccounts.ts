import {useQuery} from "@tanstack/react-query";
import {streamToString} from "../global/utils";

type Account = {
  address: string,
  balance: string,
  class_hash: string,
  private_key: string,
  public_key: string
}

const useAccounts = () => {
  return useQuery(
    ['accounts'],
    async () => {
      const data = await fetch("/api/accounts")
      if (!data.body) return [] as Account[]
      return JSON.parse(await streamToString(data.body)) as Account[]
    }
  )
}

export default useAccounts