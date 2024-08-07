import { mockIPC } from '@tauri-apps/api/mocks'
import { fireEvent } from '@testing-library/react'

import { render } from '../../../utils/testUtils/setupTest.tsx'
import TitleBarButtons from '../TitleBarButtons.tsx'

const minimizeWindow = vi.fn()
const closeWindow = vi.fn()
beforeEach(() => {
  mockIPC((command, args) => {
    if (command === 'close_main_window') {
      closeWindow()
    } else if (args.__tauriModule === 'Window' && (args.message as any)?.cmd === 'manage') {
      const { cmd } = (args.message as any).data
      if (cmd.type === 'minimize') {
        minimizeWindow()
      }
    }
  })
})

describe('TitleBarButtons', () => {
  it('should minimize the window', async () => {
    const { getByLabelText } = render(<TitleBarButtons />)
    fireEvent.click(getByLabelText('Minimize window'))
    expect(minimizeWindow).toBeCalled()
  })

  it('should close the window', async () => {
    const { getByLabelText } = render(<TitleBarButtons />)
    fireEvent.click(getByLabelText('Close window'))
    expect(closeWindow).toBeCalled()
  })
})
