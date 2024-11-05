import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PredictNavBarComponent } from './predict-nav-bar.component';

describe('PredictNavBarComponent', () => {
  let component: PredictNavBarComponent;
  let fixture: ComponentFixture<PredictNavBarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PredictNavBarComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(PredictNavBarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
