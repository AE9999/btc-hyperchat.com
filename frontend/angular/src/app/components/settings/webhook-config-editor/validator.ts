import {AbstractControl, ValidationErrors, ValidatorFn} from "@angular/forms";

function doCheckUrl(rawUrl: string) {

  if (!rawUrl) {
    return true;
  }

  try {
    const url = new URL(rawUrl);
    return url.protocol === "http:" || url.protocol === "https:";
  } catch (_) {
    return false;
  }

}

export function correctWebhookUrl(): ValidatorFn {

  return (control: AbstractControl): ValidationErrors | null => {

    return doCheckUrl(control.value) ? null
                                     :  { 'invalidWebUrl': true } ;
  };
}
