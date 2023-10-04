export const delay = (ms: number) => new Promise(res => setTimeout(res, ms));
export const manipulateBlock = (type: 'MineBlock' | 'IncreaseTime', value: number) =>
  fetch(
    "/api/block",
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Access-Control-Allow-Origin": "*",
      },
      body: JSON.stringify(
        {
          action: {
            [type]: value
          }
        }
      )
    }
  )

export const getProductionUrl = () => {
  if (import.meta.env.VITE_KATANA_URL) return import.meta.env.VITE_KATANA_URL
  const protocol = window.location.protocol
  const hostname = window.location.hostname.replace('www.', '')

  if (hostname === 'localhost') return 'http://localhost:5050'

  return`${protocol}//katana.${hostname}`
}

// Function to convert a ReadableStream to a string
export async function streamToString(readableStream: ReadableStream) {
  const textDecoder = new TextDecoder();
  const reader = readableStream.getReader();
  let result = '';

  try {
    while (true) {
      const { done, value } = await reader.read();

      if (done) {
        break; // The stream has ended
      }

      result += textDecoder.decode(value);
    }

    return result;
  } finally {
    reader.releaseLock();
  }
}

export const toOverflowValue = (originalString: string, first: number, last: number) => {
  if (originalString.length - last + first <= originalString.length) return originalString
  return originalString.substring(0, first) + "..." + originalString.substring(originalString.length - last)
}
