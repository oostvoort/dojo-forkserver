import {BlockIdentifier, GetBlockResponse} from "starknet";
import {useQuery, UseQueryOptions} from "@tanstack/react-query";
import {PROVIDER} from "../global/constants";

const useBlockInformation = (
  blockIdentifier: BlockIdentifier,
  options?:  Omit<UseQueryOptions<GetBlockResponse, unknown, GetBlockResponse, BlockIdentifier[]>, "queryKey" | "queryFn" | "initialData"> & {initialData?: () => undefined}) => {
  return useQuery(
    ['blockInformation', blockIdentifier],
    async () => PROVIDER.getBlock(blockIdentifier),
    options
  )
}

export const useLatestBlock = () => useBlockInformation("latest", { refetchInterval: 5_000 })

export default useBlockInformation