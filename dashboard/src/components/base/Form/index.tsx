export function FormTextField({
                                label,
                                value,
                                onChange
                              }: { label: string, value: string, onChange: (event: React.ChangeEvent<HTMLInputElement>) => void }) {
  return <div className='mb-4'>
    <label htmlFor={label} className='block text-white-700 font-bold mb-2'>{label}</label>
    <input
      id={label}
      type='text'
      value={value}
      onChange={onChange}
      className='shadow appearance-none border rounded w-full py-2 px-3 text-white-700 leading-tight focus:outline-none focus:shadow-outline'
    />
  </div>
}

export function FormSubmitButton({ children }: { children: React.ReactNode }) {
  return <div className='flex items-center justify-center'>
    <button type='submit'
            className='w-full bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline'>{children}
    </button>
  </div>
}

function Form({
                onSubmit,
                children
              }: { onSubmit?: (event: React.ChangeEvent<HTMLFormElement>) => void, children: React.ReactNode }) {
  return <form onSubmit={onSubmit} className='w-full max-w-sm mx-auto'>
    {children}

  </form>
}

export default Form
