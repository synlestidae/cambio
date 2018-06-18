import {CalendarDate} from '../../domain/calendar_date';

export class PersonalInfo {
    first_names: string = '';
    family_name: string = '';
    address_line_1: string = '';
    address_line_2: string = '';
    post_code: string = '';
    city: string = '';
    country: string = '';
    dob: CalendarDate = new CalendarDate()
}

export class IdentificationInfo {
    id_type: 'Passport_NZ' | 'DriversLicence_NZ' = 'Passport_NZ';
    id_number: string = '';
}

export class SignupInfo {
    email_address: string = '';
    password: string = '';
}

export type LoginPage = 'LoginInfo' | 'PersonalInfo' | 'Identification';

export class SignupState {
    info: PersonalInfo = new PersonalInfo();
    identification: IdentificationInfo = new IdentificationInfo();
    form_state: LoginPage = 'LoginInfo';
}
