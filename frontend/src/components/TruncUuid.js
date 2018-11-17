import React from 'react';
import { Link } from 'react-router-dom';
import PropTypes from 'prop-types';

// TruncUuid displays a uuid truncated with a full uuid on hover.
// It links to the given route, with the uuid appended.
const TruncUuid = props => (
  <span className="TruncUuid" title={props.uuid}>
    <Link to={`${props.route}/${props.uuid}`}>
      {props.uuid.substr(0, 8)}
    </Link>
  </span>
);
TruncUuid.propTypes = {
  uuid: PropTypes.string.isRequired,
  route: PropTypes.string.isRequired,
};
export default TruncUuid;
