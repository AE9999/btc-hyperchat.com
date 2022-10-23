import {AbstractControl, ValidationErrors, ValidatorFn} from "@angular/forms";

function isInt(value: string): boolean {
  return /^\d+$/.test(value);
}

export function amountValidator(): ValidatorFn {
  return (control: AbstractControl): ValidationErrors | null => {

    const error = {forbiddenName: {value: control.value}}

    if (!control.value) {
      return null;
    }

    if (!isInt(control.value)) {
      return error;
    }

    const amount = parseInt(control.value)

    if (amount < 1 || amount > 1000) {
      return error;
    }

    return null;
  };
}

