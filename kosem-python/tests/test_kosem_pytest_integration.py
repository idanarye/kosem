def test_basic_usage_1(kosem):
    with kosem.phase(
        kosem.Button('Click me to continue').named('this'),
        kosem.Button('No! click me instead!').named('that'),
        kosem.Textbox("Thingie"),
        kosem.Textbox("").named("data"),
    ) as phase:
        phase.wait_for_button()
        print('data', phase.read_data())

def test_basic_usage_2(kosem):
    with kosem.phase(kosem.Button('Click me to continue')) as phase:
        phase.wait_for_button()
