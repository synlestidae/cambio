export class PersonalDetails {
    first_names: string;
    family_name: string;
    address_line_1: string;
    address_line_2: string|null;
    post_code: string;
    city: string;
    country: string;
    dob: string;
    id_type: string;
    id_number: string;

    public static parse(json: any): PersonalDetails {
        throw new Error('Not implemented');
    }
}
