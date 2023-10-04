import { useState } from 'react'
import styles from './mineblock.module.css'
import {delay} from "../../../../../global/utils";

function MineBlock({ onMineBlock }: { onMineBlock: () => void }) {
  const [playing, setPlaying] = useState(false)

  const handleMouseDown = () => {
    setPlaying(true)
    delay(1_500)
      .then(() => {
        onMineBlock()
        setPlaying(false)
      })
  }

  const imgSrc = playing ? '/fork/assets/mine_block.gif' : '/fork/assets/mine_block.png'
  const className = playing ? styles.playing : styles.notPlaying

  return <div className={'flex items-center justify-center'}>
    <img
      className={className}
      src={imgSrc}
      alt={'block mine image'}
      onMouseDown={handleMouseDown}
    />
  </div>

}

export default MineBlock
