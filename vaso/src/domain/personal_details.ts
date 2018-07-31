export class PersonalDetails {
    first_names: string = '';
    family_name: string = '';
    address_line_1: string = '';
    address_line_2: string|null = null;
    post_code: string = '';
    city: string = '';
    country: string = '';
    dob: string = '';
    id_type: string = '';
    id_number: string = '';

    private constructor() {
    }

    public static parse(json: any): PersonalDetails {
        let personalDetails = new PersonalDetails();
        personalDetails.first_names = json.first_names.toString();
        personalDetails.family_name = json.family_name.toString();
        personalDetails.address_line_1 = json.address_line_1.toString();
        if (json.address_line_2) {
            personalDetails.address_line_2 = json.address_line_2.toString();
        }
        personalDetails.post_code = json.post_code.toString();
        personalDetails.city = json.city.toString();
        personalDetails.country = json.country.toString();
        personalDetails.dob = json.dob.toString();
        personalDetails.id_type = json.id_type.toString();
        personalDetails.id_number = json.id_number.toString();
        return personalDetails;
    }
}
