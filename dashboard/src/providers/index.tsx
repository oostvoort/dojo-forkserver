import { QueryClientProvider, QueryClient } from '@tanstack/react-query'

type ProviderProps = {
  children: React.ReactNode
}

const queryClient = new QueryClient()

const Providers: React.FC<ProviderProps> = ({ children }) => {
  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  )
}

export default Providers
