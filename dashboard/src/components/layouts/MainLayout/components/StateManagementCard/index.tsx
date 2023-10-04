import { useMutation } from '@tanstack/react-query'
import Button from "../../../../base/Button";
import Card from "../../../../base/Card";

const saveStateStore = {
  saveStates: []
}

type Id = string
type SaveState = { name: string, timestamp: number, state: string }
function StateManagementCard() {

  const saveStateMutation = useMutation(
    ["saveState"],
    async () => setTimeout(() => console.log("saved state"), 3_000),
    {
      onError: (e: any) => alert(e.message)
    })

  const loadStateMutation = useMutation(
    ["loadState"],
    async () => setTimeout(() => console.log("loaded state"), 3_000),
    {
      onError: (e: any) => alert(e.message)
    })

  const renameStateMutation = useMutation(async (id: Id) => {
    console.log(id)
  //   try {
  //     const saveState = saveStateStore.saveStates.get(id)
  //     if (!saveState) throw Error(`saveState ${id} not found`)
  //
  //     let input = prompt(`Enter name for save state`, ``)
  //     if (!input) return
  //
  //     saveStateStore.updateSaveState(id, {
  //       name: input,
  //       timestamp: saveState.timestamp,
  //       state: saveState.state
  //     })
  //   } catch (e: any) {
  //     if (e.message != 'unexpected response content type: application/grpc') throw e
  //   }
  // }, {
  //   onError: (e: any) => alert(e.message)
  })

  const deleteStateMutation = useMutation(async (id: Id) => {
    console.log(id)
  //   try {
  //     if (window.confirm('Delete this state?')) saveStateStore.deleteSaveSate(id)
  //   } catch (e: any) {
  //     if (e.message != 'unexpected response content type: application/grpc') throw e
  //   }
  // }, {
  //   onError: (e: any) => alert(e.message)
  })

  const resetStateMutation = useMutation(async () => {
  //   try {
  //     await forkClient.reset({}).response
  //     alert('Reset state')
  //   } catch (e: any) {
  //     if (e.message != 'unexpected response content type: application/grpc') throw e
  //   }
  // }, {
  //   onError: (e: any) => alert(e.message)
  })

  const importStateMutation = useMutation(async () => {
  //   try {
  //     const input = prompt(`Paste saveState here`, ``)
  //     if (!input) return
  //     const newSaveState = JSON.parse(input) as SaveState
  //     saveStateStore.createSaveState(newSaveState)
  //   } catch (e: any) {
  //     if (e.message != 'unexpected response content type: application/grpc') throw e
  //   }
  // }, {
  //   onError: (e: any) => alert(e.message)
  })

  const handleCopy = (id: Id) => {
    console.log(id)
    // const saveState = saveStateStore.saveStates.get(id)
    // if (!saveState) throw Error(`saveState ${id} not found`)
    // navigator.clipboard.writeText(JSON.stringify(saveState)).then(
    //   () => alert('State as hex string copied to clipboard'))
  }

  return <Card title={'Manage State'} state={'UNDER_CONSTRUCTION'}>
    <div className='flex h-80 flex-col place-content-between'>
      <div className={'max-h-48 overflow-y-auto px-2'}>
        {
          Array.from(saveStateStore.saveStates).map(([id, saveState]: [Id, SaveState], index) => {
            return (
              <div className={'flex flex-row items-center place-content-between space-x-4 mb-2'} key={index}>
                <div className={'flex flex-row items-center place-content-between space-x-4'}>
                  <SaveStateName saveState={saveState} />
                  <Button color={'transparent'} onClick={() => renameStateMutation.mutate(id)}>📝</Button>
                </div>
                <div>
                  <Button color={'transparent'} onClick={() => handleCopy(id)}>📋</Button>
                  <Button color={'transparent'} onClick={() => deleteStateMutation.mutate(id)}>❌</Button>
                  <Button isLoading={loadStateMutation.isLoading}
                          onClick={() => loadStateMutation.mutate()}>Load</Button>
                </div>
              </div>
            )
          })}
      </div>
      <div className={'flex flex-col place-content-between space-y-2'}>
        <Button width={'full'} isLoading={saveStateMutation.isLoading}
                onClick={() => saveStateMutation.mutate()}>Save</Button>
        <Button width={'full'} onClick={() => importStateMutation.mutate()}
                isLoading={resetStateMutation.isLoading}>Import</Button>
        <Button width={'full'} color='red' onClick={() => resetStateMutation.mutate()}
                isLoading={resetStateMutation.isLoading}>Reset</Button>
      </div>
    </div>
  </Card>
}

export default StateManagementCard

function SaveStateName(props: { saveState: SaveState }) {
  return <>
    {props.saveState.name && props.saveState.name !== '' ? <div>{props.saveState.name}</div> :
      <div>{new Date(props.saveState.timestamp).toLocaleString()}</div>}
  </>
}
