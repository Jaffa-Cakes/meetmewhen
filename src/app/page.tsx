'use client'

import { createEvent } from './actions'
import { EventType } from '@prisma/client'

import ExpandingInput from '@/components/expanding-input'
import { useToast } from '@chakra-ui/react'
import { useRouter } from 'next/navigation'

export default function Page() {
  const toast = useToast()
  const router = useRouter()

  async function submitForm(data: FormData) {

    let name = data.get('name') as string
    let noEarlierThan = parseInt(data.get('noEarlierThan') as string)
    let noLaterThan = parseInt(data.get('noLaterThan') as string)
    let type = (data.get('type') as string).toUpperCase() as EventType

    let event = await createEvent(name, noEarlierThan, noLaterThan, type)

    toast({
      title: 'Event Created',
      description: 'Your event has been created.',
      status: 'success',
      duration: 5000,
      isClosable: true,
    })

    router.push('/event/' + event.id)
  }

  return (
    <main>
      <div className='flex flex-col justify-evenly h-screen items-center'>
        <h1 className='text-6xl font-semibold text-center'>Meet Me When</h1>

        <form action={submitForm} className='font-medium'>
          <div className='bg-color px-5 py-2 rounded-t-xl mb-3 w-min text-4xl'>
            <ExpandingInput placeholder='Event Name' minWidth={9}/>
          </div>
            
          <div className='bg-color px-12 py-10 rounded-b-xl rounded-r-xl w-min flex items-center gap-36 text-2xl'>
            <div>
              <div className='bg-color px-4 py-2 rounded-t-xl mb-2 w-min'>
                <div className="inline-block relative">
                  <select name='type' className='bg-transparent focus:outline-none focus:ring-0 cursor-pointer'>
                    <option value='weekdays'>Weekdays</option>
                    <option value='dates'>Dates</option>
                  </select>
                </div>
              </div>
              <div className='bg-color px-3 py-3 rounded-b-xl rounded-r-xl w-96 h-72'></div>
            </div>

            <div>
              <div>
                <div className='bg-color px-4 py-2 rounded-t-xl mb-2 w-fit'>
                  <label className='block'>No Earlier Than</label>
                </div>
                <div className='bg-color px-4 py-2 rounded-b-xl rounded-r-xl mb-4 w-min'>
                  <select name='noEarlierThan' className='bg-transparent focus:outline-none focus:ring-0 cursor-pointer w-72'>
                    <option value='9'>9:00 AM</option>
                    <option value='10'>10:00 AM</option>
                  </select>
                </div>
              </div>

              <div>
                <div className='bg-color px-4 py-2 rounded-t-xl mb-2 w-fit'>
                  <label className='block'>No Later Than</label>
                </div>
                <div className='bg-color px-4 py-2 rounded-b-xl rounded-r-xl mb-4 w-min'>
                  <select name='noLaterThan' className='bg-transparent focus:outline-none focus:ring-0 cursor-pointer w-72'>
                    <option value='5'>5:00 PM</option>
                    <option value='6'>6:00 PM</option>
                  </select>
                </div>
              </div>

              <div>
                <div className='bg-color px-4 py-2 rounded-t-xl mb-2 w-fit'>
                  <label className='block'>Timezone</label>
                </div>
                <div className='bg-color px-4 py-2 rounded-b-xl rounded-r-xl mb-4 w-min'>
                  <select className='bg-transparent focus:outline-none focus:ring-0 cursor-pointer w-72'>
                    <option value='australia/melbourne'>Australia/Melbourne</option>
                    <option value='australia/perth'>Australia/Perth</option>
                  </select>
                </div>
              </div>
            </div>
          </div>

          <button type='submit' className='bg-transparent text-green-600 rounded w-full border border-green-600 mt-4 py-2 px-4 hover:bg-green-900 hover:text-color text-2xl'>Create</button>
        </form>
      </div>
    </main>
  )
}
