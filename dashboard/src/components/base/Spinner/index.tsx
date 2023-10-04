import styles from './spinner.module.css'

function Spinner() {
  return (
    <div className='flex justify-center items-center h-full'>
      <div className={styles.spinner}></div>
    </div>
  )
}

export default Spinner
