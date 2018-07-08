export function getString(obj: any, propertyName: string) {
    if (!obj.hasOwnProperty(propertyName)) {
        throw new Error(`Object does not have property '${propertyName}'.`);
    }
    let objType = typeof obj[propertyName];
    if (objType === 'string' || objType === 'number') {
    return obj[propertyName].toString();
    }
    throw new Error(`Expected property '${propertyName}' to be type string or number, got ${objType}`);
}
