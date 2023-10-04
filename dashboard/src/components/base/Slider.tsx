function Slider({
                  min = 1,
                  max = 100,
                  step = 1,
                  value,
                  onChange
                }: { min?: number, max?: number, step?: number, defaultValue?: number, value: number, onChange: (value: number) => void }) {

  function handleSliderChange(event: any) {
    onChange(Number(event.target.value))
  }

  function handleInputChange(event: any) {
    const inputValue = event.target.value
    if (inputValue === '' || (Number(inputValue) >= min && Number(inputValue) <= max)) {
      onChange(Number(inputValue))
    }
  }

  return (
    <div className='flex items-center'>
      <input
        type='range'
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={handleSliderChange}
        className='w-full mr-4 appearance-none bg-green-400 h-2 rounded-full outline-none'
      />
      <input
        type='number'
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={handleInputChange}
        className='w-20 text-center border border-green-400 rounded-md px-2 py-1'
      />
    </div>
  )
}

export default Slider
