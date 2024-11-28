import { ComponentFixture, TestBed } from '@angular/core/testing';

import { NewChatPageComponent } from './new-chat-page.component';

describe('NewChatPageComponent', () => {
  let component: NewChatPageComponent;
  let fixture: ComponentFixture<NewChatPageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [NewChatPageComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(NewChatPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
