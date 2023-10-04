import Spinner from './Spinner'

const COLOR = {
  'green': 'bg-green-500 hover:bg-green-700',
  'red': 'bg-red-500 hover:bg-red-700',
  'transparent': ''
}

const WIDTH = {
  'auto': '',
  'full': 'w-full'
}

const DEFAULT = ` text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline`

function Button({
                  onClick,
                  children,
                  isLoading = false,
                  color = 'green',
                  width = 'auto'
                }: { width?: 'auto' | 'full', isLoading?: boolean, onClick?: () => void, children?: React.ReactNode, color?: 'green' | 'red' | 'transparent' }) {


  let className = `${WIDTH[width]} ${COLOR[color]}${DEFAULT}`

  return (
    <button onClick={onClick}
            className={className}>
      {isLoading ? <Spinner /> : children}
    </button>
  )
}

export default Button
