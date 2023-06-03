'use server'

import { prisma } from '@/db';
import { EventType } from '@prisma/client'

export async function createEvent(name: string, noEarlierThan: number, noLaterThan: number, type: EventType) {
    await prisma.event.create({
        data: {
            name,
            noEarlierThan,
            noLaterThan,
            type,
        }
    })
}