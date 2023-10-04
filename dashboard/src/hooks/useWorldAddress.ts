import {useQuery} from "@tanstack/react-query";
import {streamToString} from "../global/utils";

const useWorldAddress = () => {
  return useQuery(
    ['world-address'],
    async () => {
      const data = await fetch("/api/world-address")
      if (!data.body) return ''
      return streamToString(data.body)
    }
  )
}

export default useWorldAddress