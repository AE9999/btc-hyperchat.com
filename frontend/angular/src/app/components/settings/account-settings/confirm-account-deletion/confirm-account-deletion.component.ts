import {Component, Inject, OnInit} from '@angular/core';
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";

@Component({
  selector: 'app-confirm-account-deletion',
  templateUrl: './confirm-account-deletion.component.html',
  styleUrls: ['./confirm-account-deletion.component.css']
})
export class ConfirmAccountDeletionComponent implements OnInit {

  constructor(public dialogRef: MatDialogRef<ConfirmAccountDeletionComponent>,
                  @Inject(MAT_DIALOG_DATA) public data: string,
  ) { }

  ngOnInit(): void {
  }

  onNoClick(): void {
    this.dialogRef.close();
  }


}
