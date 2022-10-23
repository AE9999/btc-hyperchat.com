import { Component, OnInit } from '@angular/core';
import {Store} from "@ngrx/store";
import {AppState} from "../../../../store/reducers";
import {TranslateService} from "@ngx-translate/core";
import {MatDialog, MatDialogRef, MAT_DIALOG_DATA} from '@angular/material/dialog';
import {ConfirmAccountDeletionComponent} from "../confirm-account-deletion/confirm-account-deletion.component";
import {MatSnackBar} from "@angular/material/snack-bar";
import {RequestDeleteAccountAction} from "../../../../store/actions/settings/delete-account.action";


@Component({
  selector: 'app-account-settings',
  templateUrl: './account-settings.component.html',
  styleUrls: ['./account-settings.component.css']
})
export class AccountSettingsComponent implements OnInit {

  tooltip_delete_account: string  = '';

  delete_confirmation: string = 'DELETE'

  constructor(private store: Store<AppState>,
              private translate: TranslateService,
              public dialog: MatDialog,
              private readonly snackBar: MatSnackBar) {

    this.translate.get('SETTINGS.TOOLTIP_DELETE_ACCOUNT')
      .subscribe(value => this.tooltip_delete_account = value);
  }

  openConfirmAccountDeletionDialog() {
    const dialogRef = this.dialog.open(ConfirmAccountDeletionComponent, {
      width: '250px',
    });

    dialogRef.afterClosed().subscribe((result: string) => {
      if (result !== this.delete_confirmation) {
        this.translate.get('SETTINGS.ACCOUNT_DELETION_CONFIRM_NO_MATCH')
          .subscribe(message =>
            this.snackBar.open(message, "",{duration: 2000})
          );
        return;
      }
      this.translate.get('SETTINGS.STARTING_DELETION_PROCESS')
        .subscribe(message => {
          this.snackBar.open(message, "", {duration: 2000})
          this.store.dispatch(RequestDeleteAccountAction({payload: {}}));
        });
    });

  }

  ngOnInit(): void {
  }

}
