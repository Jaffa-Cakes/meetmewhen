'use server'

import { prisma } from '@/db';
import { EventType } from '@prisma/client'

export async function createEvent(data: FormData) {

    let name = data.get('name') as string
    console.log(data.get('noEarlierThan') as string)
    let noEarlierThan = parseInt(data.get('noEarlierThan') as string)
    let noLaterThan = parseInt(data.get('noLaterThan') as string)
    let type = (data.get('type') as string).toUpperCase() as EventType

    await prisma.event.create({
        data: {
            name,
            noEarlierThan,
            noLaterThan,
            type,
        }
    })
}