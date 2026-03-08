import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc';

dayjs.extend(utc);

function toDayjs(value, { assumeUtc = false } = {}) {
  if (value === null || value === undefined || value === '') {
    return null;
  }

  if (dayjs.isDayjs(value)) {
    return value;
  }

  if (value instanceof Date) {
    const parsedDate = dayjs(value);
    return parsedDate.isValid() ? parsedDate : null;
  }

  if (typeof value === 'number') {
    const parsedNumber = Math.abs(value) < 1e11 ? dayjs.unix(value) : dayjs(value);
    return parsedNumber.isValid() ? parsedNumber : null;
  }

  if (typeof value === 'string') {
    const trimmed = value.trim();
    if (!trimmed) {
      return null;
    }

    const normalized = trimmed.endsWith(' UTC')
      ? trimmed.replace(' UTC', 'Z').replace(' ', 'T')
      : trimmed;

    let parsedString = (assumeUtc ? dayjs.utc : dayjs)(normalized);
    if (!parsedString.isValid() && !assumeUtc) {
      parsedString = dayjs.utc(normalized);
    }

    if (!parsedString.isValid()) {
      return null;
    }

    return assumeUtc ? parsedString.local() : parsedString;
  }

  return null;
}

export function nowMilliseconds() {
  return dayjs().valueOf();
}

export function currentDateInputValue() {
  return dayjs().format('YYYY-MM-DD');
}

export function formatDateForInput(value = dayjs()) {
  const parsedValue = toDayjs(value);
  return parsedValue ? parsedValue.format('YYYY-MM-DD') : '';
}

export function shiftDateInputValue(value, amount, unit = 'day') {
  const parsedValue = toDayjs(value);
  return parsedValue ? parsedValue.add(amount, unit).format('YYYY-MM-DD') : '';
}

export function formatDisplayDate(value) {
  const parsedValue = toDayjs(value);
  return parsedValue ? parsedValue.format('ddd, MMM D, YYYY') : '';
}

export function formatLocalDateTime(value, options = {}) {
  const parsedValue = toDayjs(value, options);
  return parsedValue ? parsedValue.format('YYYY-MM-DD HH:mm:ss') : '';
}

export function formatLocalTime(value = dayjs(), options = {}) {
  const parsedValue = toDayjs(value, options);
  return parsedValue ? parsedValue.format('HH:mm:ss') : '';
}

export function toUnixSeconds(value, options = {}) {
  const parsedValue = toDayjs(value, options);
  return parsedValue ? parsedValue.unix() : 0;
}

export function toEpochMilliseconds(value, options = {}) {
  const parsedValue = toDayjs(value, options);
  return parsedValue ? parsedValue.valueOf() : 0;
}

export function formatFilenameTimestamp(value = dayjs()) {
  const parsedValue = toDayjs(value);
  return parsedValue ? parsedValue.utc().format('YYYY-MM-DDTHH-mm-ss-SSS[Z]') : 'unknown-time';
}

export function getLocalDayIsoRange(value) {
  const parsedValue = toDayjs(value);
  if (!parsedValue) {
    return { startTime: '', endTime: '' };
  }

  const localDayStart = parsedValue.startOf('day');
  return {
    startTime: localDayStart.toISOString(),
    endTime: localDayStart.endOf('day').toISOString()
  };
}

export function getDefaultDateRange(daysBack = 0) {
  const endDate = dayjs();
  return {
    endDate: endDate.format('YYYY-MM-DD'),
    startDate: endDate.subtract(daysBack, 'day').format('YYYY-MM-DD')
  };
}

export function getCurrentYear() {
  return dayjs().year();
}