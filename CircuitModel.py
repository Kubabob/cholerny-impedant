from impedance.models.circuits import circuits
import schemdraw
import schemdraw.elements as elm

def draw_parallel_elements(drawing, elements, start_pos=(0, 0), direction='right', spacing=1, labels=None) -> schemdraw.Drawing:
    """
    Draw multiple elements in parallel using schemdraw.

    Parameters:
    - drawing: schemdraw.Drawing object to add elements to
    - elements: list of schemdraw element classes (e.g., [elm.Resistor, elm.Capacitor])
    - start_pos: tuple (x, y) for starting position
    - direction: 'right', 'left', 'up', or 'down' for the parallel orientation
    - spacing: vertical/horizontal spacing between parallel elements
    - labels: list of labels for each element (optional)

    Returns:
    - drawing: the updated schemdraw.Drawing object
    """
    if labels is None:
        labels = [None] * len(elements)

    # Determine perpendicular direction for spacing
    if direction in ['right', 'left']:
        perp_dir = 'down'
        rev_perp_dir = 'up'
    else:
        perp_dir = 'right'
        rev_perp_dir = 'left'

    # Calculate total height/width needed
    total_span = spacing * (len(elements) - 1)

    # Draw starting junction
    junction1 = elm.Dot().at(start_pos)
    drawing += junction1

    # Define the length of each branch
    branch_length = 3  # Arbitrary length for each branch

    # End position of the parallel network
    if direction == 'right':
        end_x = start_pos[0] + branch_length
        end_pos = (end_x, start_pos[1])
    elif direction == 'left':
        end_x = start_pos[0] - branch_length
        end_pos = (end_x, start_pos[1])
    elif direction == 'up':
        end_y = start_pos[1] + branch_length
        end_pos = (start_pos[0], end_y)
    else:  # down
        end_y = start_pos[1] - branch_length
        end_pos = (start_pos[0], end_y)

    # Add final junction
    junction2 = elm.Dot().at(end_pos)
    drawing += junction2

    # Calculate the middle positions for each element
    for i, (element_class, label) in enumerate(zip(elements, labels)):
        # Calculate starting and ending positions for this branch
        if direction in ['right', 'left']:
            # For horizontal orientation, branches are stacked vertically
            branch_y = start_pos[1] - total_span/2 + i * spacing
            branch_start = (start_pos[0], branch_y)
            branch_end = (end_pos[0], branch_y)
        else:
            # For vertical orientation, branches are stacked horizontally
            branch_x = start_pos[0] - total_span/2 + i * spacing
            branch_start = (branch_x, start_pos[1])
            branch_end = (branch_x, end_pos[1])

        # Draw line from main junction to branch start
        drawing += elm.Line().at(start_pos).to(branch_start)

        # Add the element in the middle of the branch
        element = element_class().label(label)
        if direction == 'right':
            drawing += element.at(branch_start).right()
        elif direction == 'left':
            drawing += element.at(branch_start).left()
        elif direction == 'up':
            drawing += element.at(branch_start).up()
        else:  # down
            drawing += element.at(branch_start).down()

        if label:
            element.label(label)

        # Draw line from element end to end junction
        drawing += elm.Line().at(element.end).to(branch_end)

        # Connect branch end to main end junction
        drawing += elm.Line().at(branch_end).to(end_pos)

    return drawing

def draw_circuit(circuit, start_pos=(0, 0), direction='right', spacing=1) -> schemdraw.Drawing:
    """
    Draw an electronic circuit based on a string representation.

    Format:
    - Component types: R (resistor), C (capacitor), L (inductor)
    - Component identifiers: numbers or strings after the component type
    - Series connection: indicated by '-'
    - Parallel connection: indicated by 'p(comp1, comp2, ...)'

    Examples:
    - "R0-R1": Two resistors in series
    - "R0-p(R1, C1)-R2": Resistor, then parallel combination of R and C, then resistor

    Parameters:
    - circuit_str: String representation of the circuit
    - start_pos: Starting position (x, y) for the circuit
    - direction: Initial direction ('right', 'left', 'up', 'down')
    - spacing: Spacing between parallel components

    Returns:
    - Drawing object with the circuit
    """

    branch_length = 3

    # Create a new drawing
    drawing = schemdraw.Drawing()

    # Dictionary to map component letters to schemdraw elements
    component_map = {
        'R': elm.RBox,
        'C': elm.Capacitor,
        'L': elm.Inductor,
        'D': elm.Diode,
        'LED': elm.LED,
        'BAT': elm.Battery,
        'SW': elm.Switch,
        'GND': elm.Ground,
        's': elm.RBox,
        'p': elm.RBox,
        'R': elm.RBox,
        'C': elm.RBox,
        'L': elm.RBox,
        'W': elm.RBox,
        'Wo': elm.RBox,
        'Ws': elm.RBox,
        'CPE': elm.RBox,
        'Q': elm.RBox,
        'La': elm.RBox,
        'G': elm.RBox,
        'Gs': elm.RBox,
        'K': elm.RBox,
        'Zarc': elm.RBox,
        'TLMQ': elm.RBox,
        'T': elm.RBox,
    }

    # Current position in the drawing
    current_pos = start_pos

    # Parse the circuit string
    parts = circuit.split('-')

    for i, part in enumerate(parts):
        part = part.strip()

        # Check if this part is a parallel combination
        if part.startswith('p(') and part.endswith(')'):
            # Extract components inside the parallel section
            parallel_str = part[2:-1]  # Remove p( and )
            parallel_components = [comp.strip() for comp in parallel_str.split(',')]

            # Convert component strings to schemdraw elements
            elements = []
            labels = []
            for comp in parallel_components:
                if not comp:  # Skip empty components
                    continue

                # Extract component type and identifier
                comp_type = comp[0]  # First character is the type (R, C, L, etc.)
                comp_id = comp[1:]   # Rest is the identifier

                if comp_type in component_map:
                    elements.append(component_map[comp_type])
                    labels.append(comp_type + comp_id)
                else:
                    print(f"Warning: Unknown component type '{comp_type}' in '{comp}'")

            # Draw the parallel combination
            if elements:
                drawing = draw_parallel_elements(
                    drawing=drawing,
                    elements=elements,
                    start_pos=current_pos,
                    direction=direction,
                    spacing=spacing,
                    labels=labels
                )

                # Update current position to the end of the parallel section
                if direction == 'right':
                    current_pos = (current_pos[0] + branch_length, current_pos[1])  # Using 3 as branch_length from draw_parallel_elements
                elif direction == 'left':
                    current_pos = (current_pos[0] - branch_length, current_pos[1])
                elif direction == 'up':
                    current_pos = (current_pos[0], current_pos[1] + branch_length)
                else:  # down
                    current_pos = (current_pos[0], current_pos[1] - branch_length)

        # Individual component
        elif part:
            comp_type = part[0]  # First character is the type (R, C, L, etc.)
            comp_id = part[1:]   # Rest is the identifier

            if comp_type in component_map:
                # Add the component
                component = component_map[comp_type]()
                component = component.label(part)

                if direction == 'right':
                    drawing.add(component.at(current_pos).right())
                elif direction == 'left':
                    drawing.add(component.at(current_pos).left())
                elif direction == 'up':
                    drawing.add(component.at(current_pos).up())
                else:  # down
                    drawing.add(component.at(current_pos).down())

                # Add label
                component.label(comp_id)

                # Update current position
                current_pos = component.end
            else:
                print(f"Warning: Unknown component type '{comp_type}' in '{part}'")

    return drawing


def draw_custom_circuit(circuit, frequency_range=(0.1, 1e5, 50), title="Custom Circuit"):
    """
    Draws a representation of a CustomCircuit from impedance.py package and plots its Bode plot
    and Nyquist plot.

    Parameters:
    - circuit: impedance.models.circuits.CustomCircuit object
    - frequency_range: tuple of (start_freq, end_freq, num_points) for the plots
    - title: Title for the plots

    Returns:
    - A dictionary containing the circuit drawing and matplotlib figures
    """
    import numpy as np
    import matplotlib.pyplot as plt
    from matplotlib.gridspec import GridSpec

    # Extract the circuit string from the CustomCircuit object
    circuit_string = circuit.circuit

    # Draw the circuit schematic
    drawing = draw_circuit(circuit_string)

    # Generate frequencies for the plots
    start_freq, end_freq, num_points = frequency_range
    frequencies = np.logspace(np.log10(start_freq), np.log10(end_freq), num=num_points)

    # Calculate impedance values
    Z = circuit.predict(frequencies)
    Z_mag = np.abs(Z)
    Z_phase = np.angle(Z, deg=True)
    Z_real = np.real(Z)
    Z_imag = np.imag(Z)

    # Create figure with subplots for magnitude and phase (Bode plot)
    bode_fig = plt.figure(figsize=(10, 8))
    gs = GridSpec(2, 1, height_ratios=[1, 1])

    # Magnitude plot
    ax1 = bode_fig.add_subplot(gs[0])
    ax1.loglog(frequencies, Z_mag, 'b-', linewidth=2)
    ax1.set_ylabel('|Z| (Ω)')
    ax1.set_title(f'{title} - Bode Plot')
    ax1.grid(True, which="both", ls="--")

    # Phase plot
    ax2 = bode_fig.add_subplot(gs[1])
    ax2.semilogx(frequencies, Z_phase, 'r-', linewidth=2)
    ax2.set_xlabel('Frequency (Hz)')
    ax2.set_ylabel('Phase (degrees)')
    ax2.set_ylim(-90, 90)
    ax2.grid(True, which="both", ls="--")

    plt.tight_layout()

    # Create Nyquist plot
    nyquist_fig = plt.figure(figsize=(8, 8))
    ax3 = nyquist_fig.add_subplot(111)
    ax3.plot(Z_real, -Z_imag, 'go-', linewidth=2, markersize=4)

    # Add frequency markers
    # Mark a few frequency points on the Nyquist plot
    marker_indices = np.linspace(0, len(frequencies)-1, 5, dtype=int)
    for idx in marker_indices:
        ax3.annotate(f"{frequencies[idx]:.1e} Hz",
                     xy=(Z_real[idx], Z_imag[idx]),
                     xytext=(5, 5),
                     textcoords='offset points',
                     fontsize=8)

    ax3.set_xlabel('$Z_{real}$ (Ω)')
    ax3.set_ylabel('$-Z_{imag}$ (Ω)')
    ax3.set_title(f'{title} - Nyquist Plot')
    ax3.grid(True)

    # Make axes equal to ensure circle arcs look like circles
    ax3.axis('equal')

    # Add origin lines
    ax3.axhline(y=0, color='k', linestyle='--', alpha=0.3)
    ax3.axvline(x=0, color='k', linestyle='--', alpha=0.3)

    plt.tight_layout()

    return {
        'drawing': drawing,
        'bode_plot': bode_fig,
        'nyquist_plot': nyquist_fig
    }

# Example usage
if __name__ == "__main__":
    from impedance.models.circuits import CustomCircuit

    # Define a custom circuit
    circuit = CustomCircuit(
        initial_guess=[1e-6, 1e-5, 1e3, 1e-6, 1],
        circuit='R0-p(R1,C1)-p(R2,C2)',
        name='Example Circuit'
    )

    draw_custom_circuit(circuit)
