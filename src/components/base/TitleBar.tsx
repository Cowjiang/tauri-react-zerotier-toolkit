import { TAURI_DRAG_REGION } from '../../constant.ts'

type TitleBarProps = {
  title?: string
  description?: string
}

function TitleBar({ title, description }: TitleBarProps) {
  return (
    <div className="w-full mt-1 mb-6 flex" {...TAURI_DRAG_REGION}>
      <div className="flex flex-col pointer-events-none">
        <h1 className="font-bold text-2xl">{title}</h1>
        <p className="mt-1 text-sm text-default-600">{description}</p>
      </div>
    </div>
  )
}

export default TitleBar
