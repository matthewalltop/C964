import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SubtypesComponent } from './subtypes.component';

describe('SubtypesComponent', () => {
  let component: SubtypesComponent;
  let fixture: ComponentFixture<SubtypesComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SubtypesComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(SubtypesComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
