% Based on the HP Photosmart 420 user manual

% swipl -s database.pl
% ?- main.

%%% PART DEFINITIONS %%%

% def_part(Part, Label, Tags, Parent).
def_part(printer, "Printer", [], null).
def_part(paper_tray, "Paper Tray", [], printer).
def_part(cartridge_door, "Cartridge Door", [], printer).
def_part(internal_battery, "Internal Battery", [battery], printer).
def_part(camera, "Camera", [], printer).
def_part(camera_battery, "Camera Battery", [battery], camera).
def_part(camera_button, "Camera Button", [button], camera).
def_part(control_panel, "Control Panel", [panel], printer).
def_part(save_button, "Save Button", [button], control_panel).
def_part(control_button, "Control Button", [button], control_panel).
def_part(on_button, "On Button", [button], control_panel).
def_part(print_button, "Print Button", [button], control_panel).
def_part(rear_panel, "Rear Panel", [panel], printer).
def_part(power_cord, "Power Cord", [port], rear_panel).
def_part(usb_port, "USB Port", [port], rear_panel).
def_part(top_panel, "Top Panel", [panel], printer).
def_part(on_light, "On Light", [light], top_panel).
def_part(status_light, "Status Light", [light], top_panel).
def_part(battery_light, "Battery Light", [light], top_panel).
def_part(remote_control, "Remote Control", [], printer).

is_part(Part) :- def_part(Part, _, _, _).
part_parent(Part, Parent) :-
    is_part(Parent),
    def_part(Part, _, _, Parent).
part_tag(Part, Tag) :-
    def_part(Part, _, Tags, _),
    member(Tag, Tags).

%%% PROBLEM CATEGORY DEFINITIONS %%%

% def_category(Category, Label).
def_category(hardware, "Hardware Problems").
def_category(printing, "Printing Problems").
def_category(bluetooth, "Bluetooth Problems").
def_category(errors, "Error Message Shown").

is_category(Category) :- def_category(Category, _).

%%% PROBLEM DEFINITIONS %%%

% def_problem(Problem, Label, Category).

% HARDWARE
def_problem(flashing(status_light), "The Status light is flashing red.", hardware).
def_problem(flashing(on_light), "The On light flashed briefly after I turned the printer off.", hardware).
def_problem(images_not_found, "The printer does not find and display the images.", hardware).
def_problem(will_not_turn_on, "The printer will not turn on.", hardware).
def_problem(printer_makes_noises, "The printer makes noises.", hardware).
def_problem(not_responding(Button), Label, hardware) :-
    part_tag(Button, button),
    label(Button, L),
    string_concat(L, " on the control panel does not respond.", Label).
def_problem(not_charging(Battery), Label, hardware) :-
    part_tag(Battery, battery),
    label(Battery, L),
    string_concat(L, " will not charge.", Label).

% PRINTING
def_problem(paper_not_feeding, "Printer does not feed into the printer correctly.", printing).
def_problem(image_at_angle, "The image is printed at an angle or is off-center.", printing).
def_problem(no_page_out, "No page came out of the printer.", printing).
def_problem(paper_jammed, "The paper jammed while printing.", printing).

% BLUETOOTH
def_problem(cannot_find_printer, "My Bluetooth device cannot find the printer.", bluetooth).
def_problem(image_printed_with_borders, "The image printed with borders.", bluetooth).

% ERRORS
def_problem(error_open_output, "Open output door or clear paper jam, then press OK.", errors).
def_problem(error_camera_connected, "Already connected to a camera.", errors).

% HELPERS
is_problem(Problem) :- def_problem(Problem, _, _).
problem_category(Problem, Category) :-
    is_category(Category),
    def_problem(Problem, _, Category).

%%% FIXES %%%

step(turn_off) :-
    ask("Is the internal battery installed in the printer?"),
    write("- Remove the battery."), nl,
    write("- Wait about 10 seconds."), nl,
    write("- Reinstall the battery.").
step(turn_off) :-
    write("- Unplug the power cord."), nl,
    write("- Wait about 10 seconds."), nl,
    write("- Plug the power cord back in.").

step(adjust_paper) :-
    write("- Remove some paper from the tray and try again."), nl,
    write("- Try loading one sheet at a time."), nl,
    write("- Make sure that the paper guide fits close to the edge.").

step(clear_jam) :-
    ask("Did the paper come part way through the front?"),
    write("Gently pull the paper towards you to remove it.").
step(clear_jam) :-
    ask("Does the paper partly stick out of the tray?"),
    write("Try removing the paper from the back of the printer.").
step(clear_jam) :-
    write("Restart the printer and it will try to eject the paper."), nl,
    step(turn_off).

fix(flashing(status_light)) :-
    ask("Is the camera connected?"),
    write("Check camera screen for instructions.").
fix(flashing(status_light)) :-
    ask("Is the printer connected to a computer?"),
    write("Check the computer monitor.").
fix(flashing(status_light)) :- step(turn_off).

fix(flashing(on_light)) :-
    write("This is a normal part of the power-down process."), nl, 
    write("It does not indicate a problem with the printer.").

fix(images_not_found) :-
    ask("Do filenames comply with the standard file naming convention?"),
    write("Adjust the filenames.").
fix(images_not_found) :- step(turn_off).

fix(will_not_turn_on) :-
    ask("Does the power source comply with power requirements of the printer?"),
    step(turn_off).
fix(will_not_turn_on) :- write("Make sure you use a suitable power source.").

fix(printer_makes_noises) :-
    write("The printer may make noises after long periods of inactivity"), nl,
    write("or when its power supply has been interrupted and then restored."), nl,
    write("This is normal operation.").

fix(not_responding(Button)) :-
    part_parent(Button, Parent),
    label(Parent, Label),
    write("An error has occured with the "), write(Label), write("."), nl,
    write("- Undock and redock the camera."), nl,
    step(turn_off).

fix(not_charging(Battery)) :-
    part_parent(Battery, Parent),
    label(Battery, BLabel),
    label(Parent, PLabel),
    write("- Open the "), write(BLabel), write(" compartment in the "), write(PLabel), write("."), nl,
    write("- Remove the "), write(BLabel), write("."), nl,
    write("- Wait about 10 seconds."), nl,
    write("- Reinstall the battery.").

fix(paper_not_feeding) :- step(adjust_paper).

fix(image_at_angle) :- step(adjust_paper).

fix(paper_jammed) :- step(clear_jam).

fix(cannot_find_printer) :-
    ask("Does the Bluetooth adapter flash?"),
    write("You may be too far from the printer."), nl,
    write("Maximum recommended distance is 10 meters.").
fix(cannot_find_printer) :- write("Make sure the wireless adapter is plugged into the camera port.").

fix(image_printed_with_borders) :- write("Your printing application may not support borderless printing.").

fix(error_open_output) :-
    step(clear_jam), nl,
    write("Press Ok to continue.").

fix(error_camera_connected) :-
    write("Only one camera can be connected at a time."), nl,
    write("Unplug the previous camera before plugging in a new one.").

% Fallback fix
fix(_) :- write("No suitable fix found :(").

%%% MENU HELPERS %%%

label(Part, Label) :- def_part(Part, Label, _, _).
label(Category, Label) :- def_category(Category, Label).
label(Problem, Label) :- def_problem(Problem, Label, _).

print_list(List) :- print_list(List, 1).
print_list([], _).
print_list([Head | Tail], N) :-
    write(N), write(". "), write(Head), nl,
    N1 is N + 1,
    print_list(Tail, N1).

select_menu(Name, List, Result) :-
    write("Choose "), write(Name), write(":"), nl,
    maplist(label, List, Labels),
    print_list(Labels),
    write("Number: "), read(Number),
    nth1(Number, List, Result),
    label(Result, ResultLabel),
    write("Selected "), write(Name), write(": "), write(ResultLabel), nl.

is_truthy('y').
is_truthy('Y').
is_truthy('yes').
is_truthy('Yes').
is_truthy('YES').

ask(Question) :-
    write(Question), write(" [y/n]: "), read(Decision),
    is_truthy(Decision).

%%% MAIN PROCEDURE %%%

main :-
    prompt(_, ''),
    findall(C, is_category(C), Categories),
    select_menu("problem category", Categories, Category),
    findall(P, problem_category(P, Category), Problems),
    select_menu("problem", Problems, Problem),
    fix(Problem), nl.
