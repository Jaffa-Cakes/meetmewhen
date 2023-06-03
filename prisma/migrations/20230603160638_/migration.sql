-- CreateEnum
CREATE TYPE "EventType" AS ENUM ('DATES', 'WEEKDAYS');

-- CreateEnum
CREATE TYPE "WeekdayName" AS ENUM ('MONDAY', 'TUESDAY', 'WEDNESDAY', 'THURSDAY', 'FRIDAY', 'SATURDAY', 'SUNDAY');

-- CreateTable
CREATE TABLE "Event" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "noEarlierThan" INTEGER NOT NULL,
    "noLaterThan" INTEGER NOT NULL,
    "type" "EventType" NOT NULL,

    CONSTRAINT "Event_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Date" (
    "id" SERIAL NOT NULL,
    "date" TIMESTAMP(3) NOT NULL,
    "eventId" TEXT NOT NULL,

    CONSTRAINT "Date_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Weekday" (
    "id" SERIAL NOT NULL,
    "weekday" "WeekdayName" NOT NULL,
    "eventId" TEXT NOT NULL,

    CONSTRAINT "Weekday_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Responder" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "eventId" TEXT NOT NULL,

    CONSTRAINT "Responder_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "DateAvailability" (
    "fromTime" INTEGER NOT NULL,
    "toTime" INTEGER NOT NULL,
    "responderId" INTEGER NOT NULL,
    "dateId" INTEGER NOT NULL,

    CONSTRAINT "DateAvailability_pkey" PRIMARY KEY ("responderId","dateId")
);

-- CreateTable
CREATE TABLE "WeekdayAvailability" (
    "fromTime" INTEGER NOT NULL,
    "toTime" INTEGER NOT NULL,
    "responderId" INTEGER NOT NULL,
    "weekdayId" INTEGER NOT NULL,

    CONSTRAINT "WeekdayAvailability_pkey" PRIMARY KEY ("responderId","weekdayId")
);

-- AddForeignKey
ALTER TABLE "Date" ADD CONSTRAINT "Date_eventId_fkey" FOREIGN KEY ("eventId") REFERENCES "Event"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Weekday" ADD CONSTRAINT "Weekday_eventId_fkey" FOREIGN KEY ("eventId") REFERENCES "Event"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Responder" ADD CONSTRAINT "Responder_eventId_fkey" FOREIGN KEY ("eventId") REFERENCES "Event"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DateAvailability" ADD CONSTRAINT "DateAvailability_responderId_fkey" FOREIGN KEY ("responderId") REFERENCES "Responder"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DateAvailability" ADD CONSTRAINT "DateAvailability_dateId_fkey" FOREIGN KEY ("dateId") REFERENCES "Date"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "WeekdayAvailability" ADD CONSTRAINT "WeekdayAvailability_responderId_fkey" FOREIGN KEY ("responderId") REFERENCES "Responder"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "WeekdayAvailability" ADD CONSTRAINT "WeekdayAvailability_weekdayId_fkey" FOREIGN KEY ("weekdayId") REFERENCES "Weekday"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
