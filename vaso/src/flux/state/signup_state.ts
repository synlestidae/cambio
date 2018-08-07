import {CalendarDate} from '../../domain/calendar_date';

export type FormState = 'LoginInfo' | 'PersonalDetails' | 'ConfirmationCode';

export class SignupState {
    formState: FormState;
    // Signup info
    emailAddress: string = '';
    password: string = '';
    passwordConfirm: string = '';
    // Personal details
    firstName: string = '';
    familyName: string = '';
    addressLine1: string = '';
    addressLine2: string = '';
    postCode: string = '';
    city: string = '';
    country: string = '';
    dob: CalendarDate = new CalendarDate()
    // ID info
    idType: 'Passport_NZ' | 'DriversLicence_NZ' = 'Passport_NZ';
    idNumber: string = '';
    identifierCode: string;
    confirmationCode: string = '';
}
