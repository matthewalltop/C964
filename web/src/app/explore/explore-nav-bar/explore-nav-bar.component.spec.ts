import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ExploreNavBarComponent } from './explore-nav-bar.component';

describe('ExploreNavBarComponent', () => {
  let component: ExploreNavBarComponent;
  let fixture: ComponentFixture<ExploreNavBarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ExploreNavBarComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ExploreNavBarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
