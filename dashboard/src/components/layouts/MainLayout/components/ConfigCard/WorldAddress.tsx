import {useState} from "react";
import useWorldAddress from "../../../../../hooks/useWorldAddress";
import {toOverflowValue} from "../../../../../global/utils";

const WorldAddress = () => {
  const [overflowView, setOverflowView] = useState(true)
  const worldAddressQuery = useWorldAddress()
  const worldAddress = worldAddressQuery.data ?? "0x00000000"
  const overflownWorldAddress = toOverflowValue(worldAddress, 7, 5)

  const handleClick = () => {
    navigator.clipboard.writeText(worldAddress)
  }
  const handleDoubleClick = () => setOverflowView(prevOverflowView => !prevOverflowView)

  return (
    <div
      className={"text-left cursor-copy"}
      onClick={handleClick}
      onDoubleClick={handleDoubleClick}
    >
      <span className={"font-bold"}> World Address: </span> { overflowView ? overflownWorldAddress : worldAddress }
    </div>
  )
}

export default WorldAddress