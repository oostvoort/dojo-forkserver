import Card from "../../../../base/Card";
import Accounts from "./Accounts";
import WorldAddress from "./WorldAddress";
const ConfigCard = () => {


  return (
    <Card title={"View Config"}>
      <WorldAddress />
      <Accounts />
    </Card>
  )
}

export default ConfigCard