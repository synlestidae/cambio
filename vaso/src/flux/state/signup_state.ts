import {CalendarDate} from '../../domain/calendar_date';

export interface PersonalInfo {
    first_names: string,
    family_name: string,
    address_line_1: string,
    address_line_2: string,
    post_code: string,
    city: string,
    country: string,
    dob: CalendarDate
}

export interface IdentificationInfo {
    id_type: 'Passport_NZ' | 'DriversLicence_NZ',
    id_number: string
}

export interface SignupInfo {
    email_address: string,
    password: string
}

export type LoginPage = 'LoginInfo' | 'PersonalInfo' | 'Identification';

export interface SignupState {
    info: PersonalInfo,
    identification: IdentificationInfo,
    form_state: LoginPage
}
