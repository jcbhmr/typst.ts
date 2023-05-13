import { Component, OnInit } from '@angular/core';
import { AppService } from './app.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass'],
})
export class AppComponent implements OnInit {
  title = 'Typst.Angular';
  artifact = new Uint8Array(0);

  constructor(private service: AppService) {}

  ngOnInit() {
    this.service.getArtifact().subscribe(artifact => {
      this.artifact = artifact;
    });
  }
}
